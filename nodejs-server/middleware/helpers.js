const db = require("../models");
const url = require('url');

exports.getHotels = function(req, res){
  db.Message.find({},{ "text":true, "user":true, "profileAccessLevel": ""})
  .then(function(entries){
    res.json(entries)
  })
  .catch(function(err){
    res.send(err)
  })
}

exports.createHotel = function(req, res){
  db.Message.create(req.body)
  .then(function(newMessage){
    res.status(201).json(newMessage)
  })
  .catch(function(err){
    res.send(err)
  })
}

exports.getHotel = function(req, res){
  db.Message.findById(req.params.hotelId)
  .then(function(foundHotel){
    res.json(foundHotel)
  })
  .catch(function(err){
    res.send(err)
  })
}

exports.updateHotel = function(req, res){
  db.Message.findOneAndUpdate({_id: req.params.hotelId} , req.body, {new: true})
  .then(function(foundHotel){
    res.json(foundHotel)
  })
  .catch(function(err){
    res.send(err)
  })
}

exports.deleteHotel = function(req, res){
  db.Message.remove({_id: req.params.hotelId})
  .then(function(foundHotel){
    res.json({message: "Was deleted successfully!"})
  })
  .catch(function(err){
    res.send(err)
  })
}


exports.getUser = function(req, res){
  db.User.findById(req.params.userId)
  .then(function(found){
    res.json(found)
  })
  .catch(function(err){
    res.send(err)
  })
}

exports.updateUser = function(req, res){
  db.User.findOneAndUpdate({_id: req.params.userId} , req.body, {new: true})
  .then(function(found){
    res.json(found)
  })
  .catch(function(err){
    res.send(err)
  })
}

exports.checkUserCredentials = function(req, res){
  db.User.findById(req.params.userId)
  .then(function(found){
    if (found.profileAccessLevel === "admin" || found.profileAccessLevel === "Admin") {
      //return next();
      console.log(found)
      res.redirect(url.format({
       pathname:"/test",
       method: "POST",
       query: {
          "a": 1,
          "b": 2,
          "valid":"your string here"
        }
     }));
    } else {
      console.log(found)
      res.redirect("https://google.com")
    }
  })
  .catch(function(err){
    res.send(err)
  })
}
