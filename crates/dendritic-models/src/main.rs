use dendritic_datasets::iris::*;
use dendritic_datasets::breast_cancer::*;
use dendritic_datasets::diabetes::*;
use dendritic_datasets::alzhiemers::*;
use dendritic_datasets::customer_purchase::*;
use dendritic_datasets::student_performance::*;
use dendritic_datasets::airfoil_noise::*;
use dendritic_regression::logistic::*;
use dendritic_regression::linear::*;
use dendritic_trees::decision_tree::*;
use dendritic_trees::decision_tree_regressor::*;
use dendritic_trees::random_forest::*; 
use dendritic_preprocessing::encoding::*;
use dendritic_preprocessing::standard_scalar::*;
use dendritic_metrics::loss::*;
use dendritic_metrics::activations::*;
use dendritic_metrics::utils::*;
use dendritic_knn::knn::*;
use dendritic_knn::distance::*; 

fn diabetes_model() {

    // load data
    let data_path = "../../datasets/data/diabetes.parquet";
    let (x_train, y_train) = load_diabetes(data_path).unwrap();

    // create logistic regression model
    let mut log_model = Logistic::new(
        &x_train,
        &y_train,
        sigmoid_vec,
        0.01
    ).unwrap();

    log_model.sgd(5000, true, 5);

    let x_test = x_train.batch(5).unwrap();
    let y_test = y_train.batch(5).unwrap();
    let y_pred = log_model.predict(x_test[30].clone());
    println!("Actual: {:?}", y_test[30]);
    println!("Prediction: {:?}", y_pred.values());

    let loss = mse(&y_test[30], &y_pred).unwrap(); 
    println!("LOSS: {:?}", loss);

}


fn breast_cancer_model() {

    // load data 
    let data_path = "../../datasets/data/breast_cancer.parquet";
    let (x_train, y_train) = load_breast_cancer(data_path).unwrap();

    // create logistic regression model
    let mut log_model = Logistic::load(
	"../data/breast_cancer",
        &x_train,
        &y_train,
        sigmoid_vec,
        0.001
    ).unwrap();

    //log_model.sgd(1500, true, 5);

    let sample_index = 450;
    let x_test = x_train.batch(5).unwrap();
    let y_test = y_train.batch(5).unwrap();
    let y_pred = log_model.predict(x_test[sample_index].clone());
    println!("Actual: {:?}", y_test[sample_index]);
    println!("Prediction: {:?}", y_pred.values());

    let loss = mse(&y_test[sample_index], &y_pred).unwrap(); 
    println!("LOSS: {:?}", loss);

    //log_model.save("../data/breast_cancer").unwrap();
}


fn iris_model() {

    // load data
    let data_path = "../../datasets/data/iris.parquet";
    let (x_train, y_train) = load_iris(data_path).unwrap();

    // encode the target variables
    let mut encoder = OneHotEncoding::new(y_train.clone()).unwrap();
    let y_train_encoded = encoder.transform();

    // create logistic regression model
    let mut log_model = MultiClassLogistic::new(
        &x_train,
        &y_train_encoded,
        softmax,
        0.1
    ).unwrap();

    log_model.sgd(500, true, 5);

    let sample_index = 100;
    let x_test = x_train.batch(5).unwrap();
    let y_test = y_train.batch(5).unwrap();
    let y_pred = log_model.predict(x_test[sample_index].clone());

    println!("Actual: {:?}", y_test[sample_index]);
    println!("Prediction: {:?}", y_pred.values());

    let loss = mse(&y_test[sample_index], &y_pred).unwrap(); 
    println!("LOSS: {:?}", loss);  
} 


fn dt_iris_model() {

    // load data
    let data_path = "../../datasets/data/iris.parquet";
    let (x_train_test, y_train_test) = load_iris(data_path).unwrap();
    let (x_train, y_train) = load_all_iris(data_path).unwrap();
    let mut model = DecisionTreeClassifier::load(
        "../data/iris_decision_tree",
        3, 3, 
        gini_impurity
    );
    model.fit(&x_train, &y_train);

    let sample_index = 100;
    let x_test = x_train_test.batch(5).unwrap();
    let y_test = y_train_test.batch(5).unwrap();
    let y_pred = model.predict(x_test[sample_index].clone());
    println!("Actual: {:?}", y_test[sample_index]);
    println!("Prediction: {:?}", y_pred.values()); 
}


fn alzheimers_model() {

    // load data
    let data_path = "../../datasets/data/alzheimers.parquet";
    let (x_train, y_train) = load_alzhiemers(data_path).unwrap();

    // create logistic regression model
    let mut log_model = Logistic::new(
        &x_train,
        &y_train,
        sigmoid_vec,
        1.0
    ).unwrap();

    log_model.train(1500, true);

    let sample_index = 100;
    let x_test = x_train.batch(5).unwrap();
    let y_test = y_train.batch(5).unwrap();
    let y_pred = log_model.predict(x_test[sample_index].clone());
    println!("Actual: {:?}", y_test[sample_index]);
    println!("Prediction: {:?}", y_pred.values());

    let loss = mse(&y_test[sample_index], &y_pred).unwrap(); 
    println!("LOSS: {:?}", loss);
}

fn airfoil_regression_tree() {

    let data_path = "../../datasets/data/airfoil_noise_data.parquet";
    let (x_train, y_train) = load_airfoil_data(data_path).unwrap();
    
    let mut model = DecisionTreeRegressor::new(30, 3, mse);
    model.fit(&x_train, &y_train);
    model.save("../data/airfoil_regression_tree").unwrap();

    let sample_index = 10;
    let x_test = x_train.batch(5).unwrap();
    let y_test = y_train.batch(5).unwrap();
    let y_pred = model.predict(x_test[sample_index].clone());

    println!("Actual: {:?}", y_test[sample_index]);
    println!("Prediction: {:?}", y_pred.values());

}

fn iris_random_forest_classifier() {

    // load data
    let data_path = "../../datasets/data/iris.parquet";
    let (x_train_test, y_train_test) = load_iris(data_path).unwrap();
    let (x_train, y_train) = load_all_iris(data_path).unwrap();

    let mut model = RandomForestClassifier::new(
        3, 3,
        100, 3,
        entropy
    );
    model.fit(&x_train, &y_train);

    let sample_index = 30;
    let x_test = x_train_test.batch(5).unwrap();
    let y_test = y_train_test.batch(5).unwrap();
    let y_pred = model.predict(x_test[sample_index].clone());
    println!("Actual: {:?}", y_test[sample_index]);
    println!("Prediction: {:?}", y_pred.values()); 


}


fn iris_knn_classifier() {

    let (x, y) = load_iris("../../datasets/data/iris.parquet").unwrap();
    let (x_train, x_test) = x.split(0, 0.80).unwrap();
    let (y_train, y_test) = y.split(0, 0.80).unwrap();

    let clf = KNN::fit(
        &x_train, 
        &y_train, 
        4, 
        euclidean
    ).unwrap();

    let predictions = clf.predict(&x_test);
    println!("Actual: {:?}", predictions.values());
    println!("Prediction: {:?}", y_test.values()); 
}


fn main() -> std::io::Result<()> {

    iris_knn_classifier(); 
    Ok(())


}
