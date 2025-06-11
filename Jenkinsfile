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
                bat 'cargo install grcov'
                bat 'set RUSTFLAGS="-Zinstrument-coverage"'
                bat 'set LLVM_PROFILE_FILE="coverage-%p-%m.profraw"'
                bat 'cargo test --verbose'
                bat 'grcov . -s . -t cobertura --branch --ignore-not-existing -o coverage/coverage.xml'
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