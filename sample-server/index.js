const express = require('express')
const app = express()
const port = 9292

app.get('/', (req, res) => {
  console.log('body: ', req.body);
  console.log('raw headers: ', req.rawHeaders);
  res.send('Hello World!')
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})
