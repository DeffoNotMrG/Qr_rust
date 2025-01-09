# 📱 QR Code Generator in Rust

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Rocket](https://img.shields.io/badge/Rocket-FF4A00?style=for-the-badge&logo=rocket&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)

Hey there! 👋 This is my little QR code generator built while exploring Rust.
It's simple but gets the job done! 

[Try It Out](#-quick-start) • [How to Use](#-api-usage) • [Help Me Improve](#-contributing)

</div>

## 🤔 What's This?

So, I was learning Rust and thought, "Hey, wouldn't it be cool to make QR codes?" 
That's how this project was born! It's a web service that turns URLs into QR codes. 
Nothing fancy, but I had fun building it and learned a ton about Rust web development along the way! 🎉

## ✨ Cool Things It Can Do

* 🎨 Makes QR codes as SVGs (they look super crisp!)
* 🔑 Has a simple API key setup (my first attempt at auth!)
* 📝 Logs stuff so you can see what's happening
* 🌐 Works with other websites (CORS stuff that I finally figured out 😅)
* ⚡️ Pretty quick (thanks, Rust! 🦀)

## 🚀 Want to Try It?

### What You'll Need

* Rust (latest stable)
* Cargo (comes with Rust)
* A few minutes of free time ⏰

### Getting Started

1. Grab the code:
```bash
git clone https://github.com/yourusername/qr-service.git
cd qr-service
```

2. Set up your secret stuff:
```bash
echo "API_KEY=whatever_you_want" > .env
echo "PORT=8080" >> .env  # optional, defaults to 8080
```

3. Let it rip!
```bash
cargo run
```

## 🎮 How to Use It

### Making QR Codes

Just send a GET request to:
```http
GET /generate?url=<the-url-you-want-as-qr>
```

Don't forget your API key (the one you put in .env):
```bash
X-API-Key: whatever_you_put_in_env
```

### Quick Test

Try this in your terminal:
```bash
curl -X GET "http://localhost:8080/generate?url=https://example.com" \
     -H "X-API-Key: your_secret_key"
```

You'll get back a nice SVG QR code! 🎨

### Health Check

Just hit the root URL to make sure it's alive:
```bash
curl http://localhost:8080/
# Should say "Hello, world!" if everything's working
```

## 🔧 Under the Hood

Here's what I'm using (had to learn all these! 📚):
* **Rocket**: Makes web stuff in Rust actually fun
* **qrcode**: The real MVP - turns URLs into QR codes
* **dotenv**: Handles the secret stuff
* **env_logger**: Helps see what's going on

## 🤔 Things to Know

* QR codes are 200x200 pixels (seemed like a good size!)
* They're black and white (keeping it simple!)
* If something goes wrong, you'll get:
  * 400 if you forgot the API key (oops!)
  * 401 if the API key is wrong (double oops!)
  * 500 if the URL breaks something (triple oops!)

## 🤝 Want to Help?

I'm still learning Rust, so I'd love any help or suggestions! If you spot something that could be better (I'm sure there's lots!):

1. Fork it
2. Make your changes
3. Send me a PR
4. Teach me something new! 🎓

## 📝 License

MIT Licensed - use it, change it, share it! Just don't blame me if something breaks 😅

## ⭐ Like It?

If this helped you learn something about Rust or QR codes, feel free to give it a star!

---

<div align="center">

Built with 🦀 Rust while drinking too much ☕ coffee

PS: If you're also learning Rust, drop by the Issues tab and say hi! 
Always happy to meet fellow learners.

</div>