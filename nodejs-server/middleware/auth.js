require("dotenv").load();
var jwt = require("jsonwebtoken");

exports.loginRequired = function(req, res, next) {

    const token = req.headers.authorization.split(" ")[1];

    jwt.verify(token, process.env.SECRET_KEY, function(err, decoded) {
      console.log("Decoded msg is: ", jwt.verify(token, process.env.SECRET_KEY))
      if (decoded) {
        next();
      } else {

        return next({ status: 401, message: "Please+Log+In First" });
      }
    });

};

exports.ensureCorrectUser = function(req, res, next) {
  try {
    const token = req.headers.authorization.split(" ")[1];
    jwt.verify(token, process.env.SECRET_KEY, function(err, decoded) {
      if (decoded && decoded.id === req.params.id) {
        return next();
      } else {
        return next({ status: 401, message: "Unauthorized" });
      }
    });
  } catch (e) {
    return next({ status: 401, message: "Unauthorized" });
  }
};

exports.ensureUserIsAdmin = function(req, res, next) {
  if (req.params.profileAccessLevel === "Admin" | "admin")
        return next();
    res.redirect("/");
};
