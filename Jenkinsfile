pipeline {
    agent any

    stages {
        stage('Setup Rust') {
            steps {
                bat 'rustup show'
                bat 'rustup default stable'
            }
        }
        stage('Build') {
            steps {
                bat 'cargo build --verbose'
            }
        }
        stage('Test') {
            steps {
                bat 'cargo test --verbose'
            }
        }
    }
}