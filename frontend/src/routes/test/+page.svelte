<script>
  let ws = new WebSocket("ws://localhost:3000/ws/hej");

  ws.onopen = () => {
    ws.onmessage = (ev) => {
      console.log(ev.data);
    };

    ws.send("hello1");
  };

  let ws2 = new WebSocket("ws://localhost:3000/ws/hej");

  ws2.onopen = () => {
    ws2.onmessage = (ev) => {
      console.log(ev.data);
    };

    ws2.send("hello2");
  };
</script>

<h1>Hello world</h1>
