use serde::{Serialize, Deserialize};
use crate::shape::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NDArray<T> {
    shape: Shape,
    size: usize,
    rank: usize,
    values: Vec<T>
}


impl<T: Default + Clone + std::fmt::Debug> NDArray<T> {

    /// Gets the rank of the current array
    pub fn rank(&self) -> usize {
        self.rank
    }

    /// Returns the shape dimensions of the array
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Get the generic values stored in the array
    pub fn values(&self) -> &Vec<T> {
        &self.values
    }
    
    /// Get the current calculated size of the contigous array
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get generic value from provided indices
    pub fn get(&self, indices: Vec<usize>) -> &T {
        &self.values[self.index(indices).unwrap()]
    }

    /// Get generic value from provided indices
    pub fn idx(&self, index: usize) -> &T {
        &self.values[index]
    }

    /// Create instance of NDArray, provide shape dimensions as parameter
    pub fn new(shape: Vec<usize>) -> Result<NDArray<T>, String> {

        let calculated_rank = shape.len(); 
        let mut calculated_size = 1; 
        for item in &shape {
            calculated_size *= item; 
        }

        Ok(Self {
            shape: Shape::new(shape),
            size: calculated_size,
            rank: calculated_rank,
            values: vec![T::default(); calculated_size],
        })
    }

    
    /// Create instance of NDArray, provide shape dimensions and array values as parameter
    pub fn array(shape: Vec<usize>, values: Vec<T>) -> Result<NDArray<T>, String> {

        let calculated_rank = shape.len(); 
        let mut calculated_size = 1; 
        for item in &shape {
            calculated_size *= item; 
        }

        if values.len() != calculated_size {
            return Err("Values don't match size based on dimensions".to_string()) 
        }

        Ok(Self {
            shape: Shape::new(shape),
            size: calculated_size,
            rank: calculated_rank,
            values: values,
        })
    }

    /// Reshape dimensions of array to new shape. Shape must match current size
    pub fn reshape(&mut self, shape_vals: Vec<usize>) -> Result<(), String> {

        if shape_vals.len() != self.rank {
            return Err("New Shape values don't match rank of array".to_string());
        }

        let mut size_validate = 1;
        for item in &shape_vals {
            size_validate *= item; 
        }

        if size_validate != self.size {
            return Err("New Shape values don't match size of array".to_string());
        }

        self.shape = Shape::new(shape_vals);
        Ok(())
    }

    /// Get contigous index of array using provided indices as parameter
    pub fn index(&self, indices: Vec<usize>) -> Result<usize, String> {

        if indices.len() != self.rank {
            return Err("Indexing doesn't match rank of ndarray".to_string());
        }

        let mut stride = 1; 
        let mut index = 0;
        let mut counter = self.rank;  
        for _n in 0..self.rank {
            let temp = stride * indices[counter-1]; 
            let curr_shape = self.shape.dim(counter-1);
            stride *= curr_shape;
            index += temp;  
            counter -= 1; 
        }

        if index > self.size-1 {
            return Err("Index out of bounds".to_string());
        }

        Ok(index)
    }

    /// Get indices from provided contigous index as parameter
    pub fn indices(&self, index: usize) -> Result<Vec<usize>, String> {

        if index > self.size-1 {
            return Err("Index out of bounds".to_string());
        }

        let mut indexs = vec![0; self.rank]; 
        let mut count = self.rank-1; 
        let mut curr_index = index; 
        for _n in 0..self.rank-1 {
            let dim_size = self.shape.dim(count);
            indexs[count] = curr_index % dim_size; 
            curr_index /= dim_size; 
            count -= 1;
        }

        indexs[0] = curr_index;
        Ok(indexs)       
    }

    /// Set index and generic value, index must be within size of array
    pub fn set_idx(&mut self, idx: usize, value: T) -> Result<(), String> {

        if idx > self.size {
            return Err("Index out of bounds".to_string());
        }

        self.values[idx] = value;
        Ok(())
    }

    /// Set generic value using provided indices. Indices must match rank of array
    pub fn set(&mut self, indices: Vec<usize>, value: T) -> Result<(), String> {

        if indices.len() != self.rank {
            return Err("Indices length don't match rank of ndarray".to_string());
        }

        let index = self.index(indices).unwrap();
        self.values[index] = value;
        Ok(())
    }

    pub fn fill(&mut self, value: T) {
        for index in 0..self.size() {
            self.values[index] = value.clone(); 
        }
    }

    /// Get rows dimension associated with multi dimensional array
    pub fn rows(&self, index: usize) -> Result<Vec<T>, String> {

        let dim_shape = self.shape.dim(0);
        let result_length = self.size() / dim_shape;
        let values = self.values();
        let mut start_index = index * result_length;
        let mut result = Vec::new();

        for _i in 0..result_length {
            let value = &values[start_index];
            result.push(value.clone());
            start_index += 1; 
        }
 
        Ok(result)

    }

    /// Get column dimension associated with multi dimensional array
    pub fn cols(&self, index: usize) -> Result<Vec<T>, String> {

        let mut result = Vec::new();
        let dim_shape = self.shape.dim(1);
        let values = self.values();
        let result_length = self.size() / dim_shape;
        let stride = dim_shape;
        let mut start = index; 

        for _i in 0..result_length {
            let value = &values[start];
            result.push(value.clone());
            start += stride; 
        }
 
        Ok(result)
    }

    pub fn batch(&self, batch_size: usize) -> Result<Vec<NDArray<T>>, String> {
       
        if batch_size == 0 || batch_size >= self.size() {
            return Err("Batch size out of bounds".to_string())
        }

        if self.rank() != 2 {
            return Err("NDArray must be of rank 2".to_string())
        }

        let dim_size = batch_size * self.shape.dim(1);
        let mut start_index = 0; 
        let mut end_index = start_index + dim_size;

        let mut batches: Vec<NDArray<T>> = Vec::new();
        
        for _item in 0..self.size() {

            if end_index >= self.size()+1 {
                break;
            }

            let temp_vec: Vec<T> = self.values()[start_index..end_index].to_vec(); 
            let ndarray_batch: NDArray<T> = NDArray::array(
                vec![batch_size, self.shape.dim(1)], 
                temp_vec.clone()
            ).unwrap();

            batches.push(ndarray_batch); 
            start_index += self.shape.dim(1); 
            end_index += self.shape.dim(1); 
             
        }

        Ok(batches) 
    }

}