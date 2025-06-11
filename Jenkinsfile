pipeline {
    agent any

    environment {
        SONAR_TOKEN = credentials('sonar-token')
        CARGO_TERM_COLOR = 'always'
    }

    stages {
        stage('Checkout') {
            steps {
                git 'https://github.com/M-Guillou/quality.git'
            }
        }

        stage('Install Rust') {
            steps {
                sh 'curl https://sh.rustup.rs -sSf | sh -s -- -y'
                sh 'source $HOME/.cargo/env'
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --verbose'
            }
        }

        stage('Test & Coverage') {
            steps {
                sh 'cargo install cargo-tarpaulin'
                sh 'cargo tarpaulin --out Lcov --output-dir ./coverage'
            }
        }

        stage('SonarCloud Analysis') {
            steps {
                withSonarQubeEnv('MySonarQubeServer') {
                    sh 'sonar-scanner'
                }
            }
        }

        stage('OWASP Check') {
            steps {
                sh './dependency-check/bin/dependency-check.sh --project your-project --scan .'
            }
        }

        stage('Quality Gate') {
            steps {
                waitForQualityGate abortPipeline: true
            }
        }
    }
}