<script>
  let ws = new WebSocket("ws://localhost:3000/ws");

  ws.onopen = () => {
    ws.onmessage = (ev) => {
      console.log(ev.data);
    };

    ws.send("hello");
  };
</script>

<h1>Hello world</h1>
