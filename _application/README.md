# Sqweebu AI Voice Assistant

Real-time voice communication with Locally hosted or Remote LLMs

Leveraging the recent advancments and general accessibility Artificial Intelligence, Navi AI Voice Assistant utilizes the Vosk API for real-time speech to text transcription. Queries are sent into a locally hosted Ollama server or remote via ChatGPT API. Finally, responses are forwarded to Microsoft Azure's Speech Synthesis service.

This application has been written and designed in Rust with Tauri to ensure cross-platform optimization and compatibility.

Initially I really wanted this to be a Privacy-focused, Self-Hosted solution although until locally hosted speech synthesis becomes more relatively accessible I believe interacting with the cloud is our best option for now.

## Features

- **Real-Time Voice Communication:** Experience seamless voice communication with Navi AI Voice Assistant, allowing for effortless interaction with your computer.

- **Locally Hosted or Remote LLMs:** Choose between locally hosted or remote Large Language Models, providing flexibility and control over your voice assistant setup.

- **Vosk API Integration:** Utilize the power of the Vosk API for lightning-fast speech-to-text transcription, ensuring accurate and responsive communication.

- **Microsoft Azure Integration:** Benefit from Microsoft Azure's Speech Synthesis service for high-quality, natural-sounding responses to your queries.

- **Cross-Platform Compatibility:** Built with Rust and Tauri, Navi AI Voice Assistant ensures cross-platform optimization and compatibility, allowing you to seamlessly use the application on various desktop operating systems.

## Installation

1. Assure Ollama server is running on port 11434

2. Sign up for Microsoft Azure
   NOTE: Microsoft offers one year free subscription to their Azure Services, after that their free tier caps out at five hours of audio a month then pay as you go charges $1 / hour.

3. Assure the following are propperly configured
   - ollama_model_name
   - ollama_alternitive_port (optional)
   - azure_api_key

## Usage

- Download the version based off your operating system found in Releases

- Launch Application
