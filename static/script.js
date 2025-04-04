const ws = new WebSocket('ws://localhost:8777/ws');

ws.onopen = function() {
  console.log('WebSocket connection opened');
}

ws.onmessage = function(event) {
    console.log(event.data);

    if(event.data.startsWith('BROADCASTM:')) {
      // console.log(event.data.substring(11));
      try {
        const [username, dataJSON] = event.data.substring(11).split('|');

        const data = JSON.parse(dataJSON);
        showData(data);

        document.getElementById('status').textContent = `${username} updated board`;
        setTimeout(() => {
          document.getElementById('status').textContent = '';
        }, 3000);
      }
      catch (e) {
        console.error('error parsing JSON:', e);
      }
    }
    else if(event.data.startsWith('enter')) {
        const username = prompt("Enter your name:");
        if(username) {
            ws.send(username);
            document.getElementById('status').textContent = `Welcome ${username}`;
            setTimeout(() => {
              document.getElementById('status').textContent = '';
            }, 3000);
        }
    }
}

ws.onerror = function(error) {
  alert.error('WebSocket error:', error);
}

ws.onclose = function() {
    console.log('WebSocket connection closed');
}


class Card {
  constructor(id, title, texts=[]) {
    this.id = id;
    this.title = title;
    this.texts = texts;
    this.element = this.createCardElement();
  }

  createCardElement() {
    const cardDiv = document.createElement('div');
    cardDiv.className = 'card';

    const title = document.createElement('h2');
    title.textContent = this.title;
    title.addEventListener('dblclick', () => {
      const newTitle = prompt('Edit title, enter new title:');
      if (newTitle) {
        sendAction({EditTitle: {card_id: this.id, text: newTitle}});
      }
    });
    cardDiv.appendChild(title);

    const textList = document.createElement('ul');
    this.texts.forEach((text, index) => {
      const listItem = document.createElement('li');
      listItem.textContent = text;

      listItem.addEventListener('dblclick', () => {
        const newText = prompt('Edit text, enter new text:');
        if (newText) {
          sendAction({EditText: {card_id: this.id, text: newText, text_index: index}});
        }

      });
      textList.appendChild(listItem);
    });
    cardDiv.appendChild(textList);

    const addTextButton = document.createElement('button');
    addTextButton.textContent = '+';
    addTextButton.addEventListener('click', () => {
      sendAction({AddText: {card_id: this.id, text: 'New text'}});
    });
    cardDiv.appendChild(addTextButton);



    const removeButton = document.createElement('button');
    removeButton.textContent = '-';
    removeButton.addEventListener('click', () => {
      sendAction({RemoveCard: {id: this.id}});
    });
    cardDiv.appendChild(removeButton);

    return cardDiv;
  }
}



function sendAction(msg) {
  ws.send(JSON.stringify(msg));
  setTimeout(fetchData, 100);
}



function fetchData() {
    fetch('http://localhost:8777/board')
    .then(response => response.json())
    .then(data => showData(data))
    .catch(error => console.error('Error fetching data:', error));

    // console.log(data);

    // showData(data);
}

function showData(data) {
  const cardsDiv = document.getElementById('cards');
  cardsDiv.innerHTML = '';
  
  data.cards.forEach(card => {
    const newCard = new Card(card.id, card.title, card.texts);
    cardsDiv.appendChild(newCard.element);
  });
}

document.getElementById('add-card').addEventListener('click', () => {
  sendAction({AddCard: null});
  setTimeout(fetchData, 100);
});

window.onload = fetchData;