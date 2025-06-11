pipeline {
    agent {
        docker { image 'rust:latest' }
    }

    stages {
        stage('Build') {
            steps {
                sh 'cargo build --verbose'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --verbose'
            }
        }
    }
}