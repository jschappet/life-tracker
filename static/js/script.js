console.log("Life Tracker JS Loaded");

function setToken(token) {
  localStorage.setItem('auth_token', token);
  console.log('Token set:', token);
  //document.getElementById('api-actions').style.display = 'block';
}

async function callApi(endpoint) {
  const token = localStorage.getItem('auth_token');
  try {
    const response = await fetch(`/tracker/api/page/${endpoint}`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    const text = await response.text();
    document.getElementById('api-result').textContent =
      `${endpoint} response: ${response.status} ${text}`;
  } catch (e) {
    document.getElementById('api-result').textContent =
      `Error: ${e.message}`;
  }
}

function callAdmin() { callApi('tasks'); }
function callManager() { callApi('projects'); }

function logout() {
  localStorage.removeItem('auth_token');
  document.getElementById('api-actions').style.display = 'none';
  document.getElementById('api-result').textContent = '';
  document.getElementById('result').textContent = 'Logged out';
}

function getBearerTokenFromCookie() {
  const cookie = document.cookie.split('; ').find(row => row.startsWith('jwt_token='));
  return cookie ? cookie.split('=')[1] : null;
}

setToken(getCookie('jwt_token'));

// Function to get a new photo from Unsplash in a nature theme and set it as the background image
async function setBackgroundImage() {
  try {
    const response = await fetch(`https://api.unsplash.com/photos/random?query=nature&client_id=VYaoddg73fuC3qzqxcHQFZeynSQLPpuPviwSk35rxsY`);
    const data = await response.json();
    if (data && data.urls && data.urls.full) {
      document.body.style.backgroundImage = `url(${data.urls.full})`;
      document.body.style.backgroundSize = 'cover';
      document.body.style.backgroundPosition = 'center';
    }
  } catch (error) {
    console.error('Error fetching background image:', error);
  }
}

// Call the function to set the background image
setBackgroundImage();

function getCookie(name) {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) return parts.pop().split(';').shift();
  return null;
}

// Check if Form Exists First
if (document.getElementById('task-form')) {
  document.getElementById('task-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const formData = new FormData(e.target);
    const data = Object.fromEntries(formData.entries());
    data.user_id = parseInt(data.user_id, 10); // Convert user_id to an integer
    const token = localStorage.getItem('auth_token');
    console.log(token);
    try {
      const response = await fetch('/api/tasks', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify(data)
      });
      const result = await response.json();
      if (response.ok) {
        alert('Task added successfully');
        document.querySelector('.task-form').style.display = 'none';
        location.reload(); // Reload the page to show the new task
      } else {
        alert(`Error: ${result.message}`);
      }
    } catch (error) {
      alert(`Error: ${error.message}`);
    }
  });
}



function toggleTask(button) {
  const isStarted = button.dataset.state === 'started';
  if (isStarted) {
    button.innerText = 'Start Task';
    button.classList.remove('bg-sage-dark');
    button.classList.add('bg-sage-light');
    button.dataset.state = 'stopped';
    button.setAttribute('hx-post', './end-task');
  } else {
    button.innerText = 'Stop Task';
    button.classList.remove('bg-sage-light');
    button.classList.add('bg-sage-dark');
    button.dataset.state = 'started';
    button.setAttribute('hx-post', './start-task');
  }
}

function markTaskStopped(button, task_id) {
  const notes = prompt('Please add any notes for this task:');
  if (notes !== null) {
    const taskElement = button.parentElement;

    event.preventDefault();
    const token = localStorage.getItem('auth_token');

    fetch('/tracker/api/tt-update-task', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ task_id: task_id, notes: notes, status: 'completed' })
    })
      .then(response => response.json())
      .then(data => {
        console.log('Success:', data);
        if (data.status === 'success') {
          taskElement.classList.add('line-through', 'text-sage-muted');
          button.remove();
          console.log('Notes:', notes);
        }
      })
      .catch(error => {
        console.error('Error:', error);
      });
  }
}

function addTaskToList(task) {
  const resultsDiv = document.getElementById('results');
  const taskElement = document.createElement('div');
  taskElement.className = 'flex justify-between items-center bg-sage-light p-2 rounded-lg mb-2';
  console.log(task);
  taskElement.innerHTML = `<li class="list-row" id="task-${task.id}">
    <div>
      <img class="size-10 rounded-box" src="https://via.placeholder.com/40" alt="Task Icon"/>
    </div>
    <div>
      <div class="font-semibold">${task.title}</div>
      <div class="text-xs opacity-60">${task.description}</div>
    </div>
    <button class="btn btn-square btn-ghost" onclick="markTaskStopped(this, ${task.id})">
      <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
          <path d="M6 3L20 12 6 21 6 3z"></path>
        </g>
      </svg>
    </button>
  </li>
`;


// TODO - Fix this heading 
  // Insert after the heading
  if (resultsDiv.children.length > 1) {
    resultsDiv.children[1].after(taskElement);
  } else {
    resultsDiv.appendChild(taskElement);
  }
}

function submitTask(event) {
  event.preventDefault();
  const taskInput = document.getElementById('taskInput');
  const token = localStorage.getItem('auth_token');

  fetch('/tracker/api/submit-task', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`
    },
    body: JSON.stringify({ taskInput: taskInput.value, "status": "in_progress" })
  })
    .then(response => response.json())
    .then(data => {
      console.log('Success:', data);
      if (data.status === 'success') {
        addTaskToList({ title: taskInput.value, id: data.task.task_id });
        taskInput.value = ''; // Clear the input
      }
    })
    .catch(error => {
      console.error('Error:', error);
    });
}

function selectSuggestion(suggestion) {
  const taskInput = document.getElementById('taskInput');
  taskInput.value = suggestion;
  document.getElementById('suggestions').innerHTML = ''; // Clear suggestions
}

// Handle login response and store token in localStorage
async function handleLoginResponse(response) {
  const result = await response.json();
  if (response.ok) {
    localStorage.setItem('auth_token', result.token);
    alert('Login successful');
    window.location.href = '/dashboard';
  } else {
    alert(`Error: ${result.message}`);
  }
}

