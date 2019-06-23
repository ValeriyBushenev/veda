/**
  Update service for individuals that were changed on server
*/

veda.Module(function (veda) { "use strict";

  veda.UpdateService = function () {

    var self = this;

    // Singleton pattern
    if (veda.UpdateService.prototype._singletonInstance) {
      return veda.UpdateService.prototype._singletonInstance;
    }

    this.list = {};
    var buffer = [];
    var socketDelay = 1000;
    var socketTimeout;
    var reconnectDelay = 5000 + Math.round(Math.random() * 5000);

    return veda.UpdateService.prototype._singletonInstance = initSocket();

    function initSocket() {
      return veda.Backend.get_individual(veda.ticket, "cfg:ClientUpdateServicePort").then(function (ccusPortCfg) {
        var ccusPort = ccusPortCfg["rdf:value"] && ccusPortCfg["rdf:value"][0].data,
            protocol = location.protocol === "http:" ? "ws:" : "wss:",
            port = ccusPort || ( protocol === "ws:" ? 8088 : 443 ),
            address = protocol + "//" + location.hostname + ":" + port + "/ccus",
            socket = new WebSocket(address);

        socket.onopen = openedHandler;
        socket.onclose = closedHandler;
        socket.onmessage = messageHandler;
        socket.receiveMessage = receiveMessage;
        socket.sendMessage = sendMessage;
        self.socket = socket;
        return self;
      });
    }

    function sendMessage (msg) {
      var socket = this;
      if (msg === "=" || msg === "-*" || msg.indexOf("ccus") === 0) {
        if (socket.readyState === 1) {
          socket.send(msg);
          //console.log("client -> server:", msg);
        }
        return;
      }
      buffer.push(msg);
      if ( !socketTimeout ) {
        socketTimeout = setTimeout(function () {
          var message = buffer.join(",");
          if (socket.readyState === 1) {
            socket.send(message);
            //console.log("client -> server:", message);
          }
          buffer = [];
          socketTimeout = undefined;
        }, socketDelay);
      }
    }

    function receiveMessage(msg) {
      //console.log("server -> client:", msg);
      var uris = msg.indexOf("=") === 0 ? msg.substr(1) : msg;
      if (uris.length === 0) {
        return;
      }
      uris = uris.split(",");
      for (var i = 0; i < uris.length; i++) {
        try {
          var tmp = uris[i].split("=");
          var uri = tmp[0];
          if ( !uri ) {
            continue;
          }
          var updateCounter = parseInt(tmp[1]);
          var individual = new veda.IndividualModel(uri);
          if ( individual.hasValue("v-s:updateCounter", updateCounter) ) { continue; }
          if (self.list[uri]) {
            self.list[uri].updateCounter = updateCounter;
          }
          if (self.list[uri].action) {
            self.list[uri].action.call(individual, updateCounter); // Call action
          } else if (updateCounter !== 0) {
            individual.reset(); // Default action
          }
        } catch (error) {
          console.log("error: individual update service failed", error);
        }
      }
    }

    function openedHandler(event) {
      console.log("client: websocket opened", event.target.url);
      this.sendMessage("ccus=" + veda.ticket);
      self.restore();
    }

    function messageHandler(event) {
      var msg = event.data;
      this.receiveMessage(msg);
    }

    function closedHandler(event) {
      console.log("client: websocket closed", event.target.url, "| re-connect in", reconnectDelay / 1000, "sec");
      setTimeout(function () {
        initSocket();
      }, reconnectDelay);
    }

  };

  var proto = veda.UpdateService.prototype;

  proto.subscribe = function (uri, action) {
    var self = this;
    if ( this.list[uri] ) {
      ++this.list[uri].subscribeCounter;
    } else {
      var individual = new veda.IndividualModel(uri);
      individual.load().then(function (individual) {
        var updateCounter = individual.hasValue("v-s:updateCounter") ? individual.get("v-s:updateCounter")[0] : 0;
        self.list[uri] = {
          subscribeCounter: 1,
          updateCounter: updateCounter
        };
        if (action) {
          self.list[uri].action = action;
        }
        self.socket.sendMessage("+" + uri + "=" + updateCounter);
      });
    }
  };

  proto.unsubscribe = function (uri) {
    if ( !uri ) {
      this.list = {};
      this.socket.sendMessage("-*");
    } else if ( this.list[uri] && this.list[uri].subscribeCounter > 1) {
      --this.list[uri].subscribeCounter;
    } else {
      delete this.list[uri];
      this.socket.sendMessage("-" + uri);
    }
  };

  proto.restore = function () {
    this.socket.sendMessage("-*");
    var subscribeMsg = [];
    for (var uri in this.list) {
      this.socket.sendMessage("+" + uri + "=" + this.list[uri].updateCounter);
    }
  };

});
