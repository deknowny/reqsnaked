# Cookies
You can automatically store and reuse cookies you recieved in a response using `#!python store_cookies=True` parameter in `Client`. It will keep them in a jar.

```python
client = reqsnaked.Client(store_cookie=True)
```

!!! Todo
    Cookies wrapper is not implement in beter way now, it comes later. Now you should pass cookies by string into cookie header and read it using some kind of `http.cookiejar`
