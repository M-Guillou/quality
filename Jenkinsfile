pipeline {
    agent any

    environment {
        SONAR_TOKEN = credentials('sonar-token')
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Build & Test') {
            steps {
                script {
                    // Run tests and capture exit code
                    def testResult = bat(returnStatus: true, script: 'mvn clean verify')

                    // Mark build as unstable if tests failed
                    if (testResult != 0) {
                        echo "Tests failed but continuing to next stages"
                        currentBuild.result = 'UNSTABLE'
                    }
                }
            }
        }

        stage('SonarCloud Analysis') {
            steps {
                withCredentials([string(credentialsId: 'sonar-token', variable: 'SONAR_TOKEN')]) {
                    withSonarQubeEnv('SonarCloud') {
                        bat "mvn sonar:sonar -Dsonar.login=${env.SONAR_TOKEN}"
                    }
                }
            }
        }

        stage('OWASP Dependency Check') {
            steps {
                bat 'mvn org.owasp:dependency-check-maven:check'
            }
        }

        stage('Quality Gate') {
            steps {
                timeout(time: 2, unit: 'MINUTES') {
                    waitForQualityGate abortPipeline: true
                }
            }
        }
    }
}
