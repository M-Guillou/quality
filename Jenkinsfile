pipeline {
    agent any

    stages {
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