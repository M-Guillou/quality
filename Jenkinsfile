pipeline {
    agent any

    environment {
        SONAR_TOKEN = credentials('sonar-token')
        CARGO_TERM_COLOR = 'always'
    }

    stages {
        stage('Checkout') {
            steps {
                git branch: 'main', url: 'https://github.com/M-Guillou/quality.git'
            }
        }

        stage('Install Rust') {
            steps {
                bat '''
                curl -o rustup-init.exe https://win.rustup.rs/x86_64
                rustup-init.exe -y
                '''
            }
        }

        stage('Build') {
            steps {
                bat 'cargo build --verbose'
            }
        }

        stage('Test & Coverage') {
            steps {
                bat 'cargo install cargo-tarpaulin'
                bat 'cargo tarpaulin --out Xml --output-dir coverage'
            }
        }

        stage('SonarCloud Analysis') {
            steps {
                withSonarQubeEnv('SonarCloud') {
                    bat 'sonar-scanner'
                }
            }
        }

        stage('OWASP Check') {
            steps {
                bat './dependency-check/bin/dependency-check.sh --project your-project --scan .'
            }
        }

        stage('Quality Gate') {
            steps {
                waitForQualityGate abortPipeline: true
            }
        }
    }
}