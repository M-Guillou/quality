pipeline {
    agent any

    environment {
        SONAR_TOKEN = credentials('sonar-token-id')
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        stage('Setup Rust') {
            steps {
                bat 'rustup default stable || rustup install stable && rustup default stable'
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
        stage('Coverage') {
            steps {
                bat 'cargo install cargo-tarpaulin || echo "tarpaulin already installed"'
                bat 'cargo tarpaulin --out Lcov --output-dir coverage'
            }
        }
        stage('SonarQube Analysis') {
            steps {
                withSonarQubeEnv('SonarQube') {
                    bat 'sonar-scanner -Dsonar.projectKey=your_project_key -Dsonar.sources=. -Dsonar.coverageReportPaths=coverage/lcov.info'
                }
            }
        }
        stage('OWASP Scan') {
            steps {
                bat 'dependency-check.sh --project YourProjectName --scan . --format HTML --out reports/dependency-check-report.html'
            }
        }
    }
}