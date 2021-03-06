require("dotenv").config();
const express = require("express");
const app = express();
const cors = require("cors");
const bodyParser = require("body-parser");
const errorHandler = require("./handlers/error");
const authRoutes = require("./routes/auth");
const messagesRoutes = require("./routes/messages");
const { loginRequired, ensureCorrectUser } = require("./middleware/auth");
const helpers = require("./middleware/helpers");
const db = require("./models");
const PORT = 1331;
const routerTest = express.Router();

app.use(cors());
app.use(bodyParser.json());

app.use("/api/auth", authRoutes);
app.use(
  "/api/users/:id/messages",
  loginRequired,
  ensureCorrectUser,
  messagesRoutes
);

app.use(express.static(__dirname + "/views"));

app.get("/", function(req, res){
  console.log(req.headers);
  res.sendFile("index.html")
})

app.get("/test", helpers.getHotels)
app.post("/test", helpers.createHotel)

app.route("/user/:userId")
  .get(helpers.getUser)
  .put(helpers.updateUser)

app.route("/user/isadmin/:userId")
    .get(helpers.checkUserCredentials)

app.route("/test/:hotelId")
      .get(helpers.getHotel)
      .put(helpers.updateHotel)
      .delete(helpers.deleteHotel)

app.get("/api/messages/showall", async function(req, res, next) {
  try {
    let messages = await db.Message.find()
      .sort({ createdAt: "desc" })
      .populate("user", {
        username: true,
        profileAccessLevel: true
      });
    return res.status(200).json(messages);
  } catch (err) {
    return next(err);
  }
});

app.get("/api/messages", loginRequired, async function(req, res, next) {
  try {
    let messages = await db.Message.find()
      .sort({ createdAt: "desc" })
      .populate("user", {
        username: true,
        profileAccessLevel: true
      });
    return res.status(200).json(messages);
  } catch (err) {
    return next(err);
  }
});

app.use(function(req, res, next) {
  let err = new Error("Not Found");
  err.status = 404;
  next(err);
});

app.use(errorHandler);

app.listen(PORT, function() {
  console.log(`Server is starting on port ${PORT}`);
});
