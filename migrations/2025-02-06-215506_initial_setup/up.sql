
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  NOT NULL 
);

INSERT INTO users (id, username, email, password_hash) 
values (0,"Not Assigned","nobody@nowhere.com", "" );

CREATE TABLE goals (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    due_date DATE,
    status TEXT CHECK(status IN ('pending', 'in_progress', 'completed')) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  NOT NULL, 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE project_goals (
    project_id INTEGER NOT NULL,
    goal_id INTEGER NOT NULL,
    PRIMARY KEY (project_id, goal_id),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE CASCADE
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL DEFAULT 0,
    project_id INTEGER,
    title TEXT NOT NULL,
    description TEXT,
    due_date DATE,
    status TEXT CHECK(status IN ('pending', 'in_progress', 'completed')) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
);

CREATE TABLE daily_todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    date DATE NOT NULL DEFAULT (DATE('now')),
    completed BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE TABLE rewards (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    user_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    points INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
