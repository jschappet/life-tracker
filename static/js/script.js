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
  const taskElement = document.createElement('li');
  taskElement.className = "list-row justify-between";
  taskElement.id = "task-${task.id}";
  console.log(task);
  taskElement.innerHTML = `<div>
        <div class="font-semibold">${task.title}</div>
        <div class="text-xs opacity-60">&nbsp;</div>
      </div>
      <button class="btn btn-square btn-ghost align-top" onclick="markTaskStopped(this, ${task.id})">
        <svg xmlns="http://www.w3.org/2000/svg" id="Layer_1" data-name="Layer 1" 
          viewBox="0 0 24 24" 
          stroke-width=".6" stroke="currentColor" class="size-[1.5em]">
          <path
            d="m22.389,5.418l-3.808-3.808c-1.04-1.039-2.42-1.61-3.889-1.61h-5.385c-1.469,0-2.85.572-3.889,1.611l-3.808,3.808c-1.039,1.04-1.61,2.42-1.61,3.889v5.385c0,1.469.572,2.85,1.611,3.889l3.808,3.808c1.04,1.039,2.42,1.61,3.889,1.61h5.385c1.469,0,2.85-.572,3.889-1.611l3.808-3.808c1.039-1.04,1.61-2.42,1.61-3.889v-5.385c0-1.469-.572-2.85-1.611-3.889Zm-1.389,9.274c0,.667-.26,1.296-.732,1.768l-3.808,3.807c-.472.472-1.101.732-1.768.732h-5.385c-.667,0-1.296-.26-1.768-.732l-3.807-3.808c-.472-.472-.732-1.101-.732-1.768v-5.385c0-.667.26-1.296.732-1.768l3.808-3.807c.472-.472,1.101-.732,1.768-.732h5.385c.667,0,1.296.26,1.768.732l3.807,3.808c.472.472.732,1.101.732,1.768v5.385Zm-3.407-6.22c.567.604.538,1.553-.065,2.12l-4.145,3.896c-.699.7-1.629,1.052-2.565,1.052-.946,0-1.898-.36-2.622-1.084l-1.795-1.938c-.563-.608-.526-1.557.082-2.12.608-.563,1.557-.527,2.12.082l1.755,1.896c.23.229.668.229.938-.042l4.178-3.928c.604-.569,1.553-.539,2.12.065Z" />
        </svg>
      </button>
  `;

  if (resultsDiv.children.length > 1) {
    resultsDiv.children[0].after(taskElement);
  } else {
    resultsDiv.appendChild(taskElement);
  }
}

let selectedTags = [];

function toggleTag(tagId) {
  console.log("toggling tag: ", tagId);
  const index = selectedTags.indexOf(tagId);
  const tagElement = document.querySelector(`.tag[data-id="${tagId}"]`);
  if (index > -1) {
    selectedTags.splice(index, 1);
    tagElement.classList.remove('badge-primary');
    tagElement.classList.add('badge-neutral');
  } else {
    selectedTags.push(tagId);
    tagElement.classList.remove('badge-neutral');
    tagElement.classList.add('badge-primary');
  }
}

function addNewTag() {
  const token = localStorage.getItem('auth_token');

  const tagName = prompt("Enter new tag name:");
  if (tagName) {
    fetch('/tracker/api/tags', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ name: tagName })
    })
    .then(response => response.json())
    .then(tag => {
      const tagElement = document.createElement('span');
      tagElement.className = 'tag bg-gray-500';
      tagElement.dataset.id = tag.id;
      tagElement.innerText = tag.name;
      tagElement.onclick = () => toggleTag(tag.id);
      document.getElementById('tags').appendChild(tagElement);
    });
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
    body: JSON.stringify({ taskInput: taskInput.value, tags: selectedTags, "status": "in_progress" })
  })
    .then(response => response.json())
    .then(data => {
      console.log('Success:', data);
      if (data.status === 'success') {
        addTaskToList({ title: taskInput.value, id: data.task.task_id });
        taskInput.value = ''; // Clear the input
        selectedTags = []; // Clear selected tags
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


document.body.addEventListener("htmx:configRequest", function (evt) {
  const authCookie = localStorage.getItem('auth_token');
  if (authCookie) {           // Set the authentication header for all future requests                
    evt.detail.headers["Authorization"] = "Bearer " + authCookie;
  }
});