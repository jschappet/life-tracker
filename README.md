# Life Tracker

Life Tracker is a web application designed to help you manage and track your tasks efficiently. It allows you to start, stop, and submit tasks, and provides a visual representation of active tasks. The application also features an autocomplete functionality for task input and a dynamic background image fetched from Unsplash.

## Features

- **Task Management**: Start, stop, and submit tasks.
- **Autocomplete**: Provides suggestions as you type your task.
- **Dynamic Background**: Fetches a random nature-themed background image from Unsplash.
- **Responsive Design**: Optimized for both desktop and mobile devices.

## Technologies Used

- **Frontend**: HTML, CSS (TailwindCSS), JavaScript
- **Backend**: Rust, Actix
- **Database**: Sqlite
- **API**: Unsplash API for background images

## Installation

### Node.js and Express

1. Clone the repository:
   ```bash
   git clone https://github.com/jschappet/life-tracker.git
   cd life-tracker
   ```

2. Install dependencies:
   ```bash
   npm install
   ```


### Rust and Actix

1. Ensure you have Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/).

3. Build and run the Actix server:
   ```bash
   cargo run
   ```

4. The Actix server will start on `http://localhost:8080`.

## Usage

- **Start Task**: Enter a task in the input field and click "Start Task".
- **Stop Task**: Click the "Stop" button next to an active task to stop it.
- **Submit Task**: Click the "Submit" button to submit the task.
- **Autocomplete**: Suggestions will appear as you type in the task input field.

## Project Structure

```
life-tracker/
├── static/
│   ├── css/
│   │   └── output.css
│   ├── js/
│   │   └── script.js
├── templates/
│   └── time-tracker.hbs
├── src/
│   └── main.rs
│   Cargo.toml
├── .env
├── package.json
├── README.md
└── server.js
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Unsplash](https://unsplash.com) for providing the background images.
- [TailwindCSS](https://tailwindcss.com) for the CSS framework.


