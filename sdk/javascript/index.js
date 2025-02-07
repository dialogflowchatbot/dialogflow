class MySDK {
    constructor(apiKey) {
        this.apiKey = apiKey;
    }
 
    async fetchData(url) {
        const response = await fetch(`https://api.example.com/${url}`, {
            headers: {
                'Authorization': `Bearer ${this.apiKey}`
            }
        });
        return await response.json();
    }
 
    async postData(url, data) {
        const response = await fetch(`https://api.example.com/${url}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.apiKey}`
            },
            body: JSON.stringify(data)
        });
        return await response.json();
    }
}
 
module.exports = MySDK;
