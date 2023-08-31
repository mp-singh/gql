import http from 'k6/http';

const query = `
query {
    apiVersion
    users {
      name
      color {
        id
        name
      }
    }
  }  
`;

const mutation = `
  mutation addUser {
    addUser(
      userInput: "JASS", 
      phoneInput: {number: "123456789", phoneType: HOME}, 
      colorInput: {name: "BLACK"})
  }
`;

const headers = {
  'Content-Type': 'application/json',
};

export default function () {
    http.post(
      'http://localhost:8080/graphql',
      JSON.stringify({ query: query }),
      { headers },
    );
}
