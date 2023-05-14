import ws from "k6/ws";
import { check, sleep } from "k6";

export let options = {
  stages: [
    { duration: "2m", target: 10 }, // ramp up to 10 connections
    { duration: "2m", target: 10 }, // stay at 10 connections
    { duration: "2m", target: 100 }, // ramp up to 100 connections
    { duration: "2m", target: 100 }, // stay at 100 connections
    // Add more stages for 1000, 10000, etc.
  ],
};

export default function () {
  const url = "ws://127.0.0.1:9001";
  const params = { tags: { my_tag: "hello" } };

  const res = ws.connect(url, params, function (socket) {
    check(socket, {
      "WebSocket connection established": (s) => s.readyState === 1,
    });

    let messageCount = 0;
    const startTime = new Date().getTime();

    while (new Date().getTime() - startTime < 1000) {
      socket.send(JSON.stringify({ type: "message", content: "Test message" }));
      messageCount++;
    }

    socket.close();

    const messagesPerSecond = messageCount / 1;
    console.log(`Messages per second: ${messagesPerSecond}`);
  });

  check(res, { "Status is 101": (r) => r && r.status === 101 });
  sleep(1);
}
