# OneLLM:

## TODO List:
1. Deduct balance upon API Request IMPORTANT
1. Add STRIPE integration IMPORTANT
1. ApiKey in website should not be placholders. 
1. Make Proposal for 5k for testing and Beta launch
1. Use ~~SRP2~~ TLS
1. Fix up website (mostly done, just a few things left)
1. 2FA fixed (delay to after launch)
1. Add rate limits

## Testing commands:
### DeepSeek:
```zsh
curl -X POST http://127.0.0.1:3000/api \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer oa-28366171116533564198" \
  -d '{
    "endpoint": "https://api.deepseek.com/chat/completions",
    "model": "DeepSeek-Chat",
    "messages": [
      { "role": "system", "content": "You are a helpful assistant." },
      { "role": "user", "content": "Hello!" }
    ],
    "frequency_penalty": 0,
    "max_tokens": 2048,
    "presence_penalty": 0,
    "stream": false,
    "temperature": 1,
    "top_p": 1,
    "logprobs": false,
    "top_logprobs": null
  }'
```

### Gemini (API Key auth isn't working, but parsing and everything else is working): 

```zsh
curl -X POST http://127.0.0.1:3000/api \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer oa-28366171116533564198" \
  -d '{
    "endpoint": "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent",
    "model": "1.5-Flash",
    "temperature": 1,
    "top_p": 1,
    "top_k": 40,
    "candidate_count": 1,
    "max_tokens": 2048,
    "stream": false,
    "messages": [],
    "contents": [
      {
        "role": "user",
        "parts": [
          { "text": "Hello!" }
        ]
      }
    ],
    "generation_config": {
      "temperature": 1,
      "top_p": 1,
      "top_k": 40,
      "candidate_count": 1,
      "max_output_tokens": 2048,
      "stop_sequences": []
    },
    "safety_settings": [
      { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_LOW_AND_ABOVE" },
      { "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_LOW_AND_ABOVE" },
      { "category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_LOW_AND_ABOVE" },
      { "category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_LOW_AND_ABOVE" }
    ]
  }'
  ```

### OpenAI

### Claude
