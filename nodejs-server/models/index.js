const mongoose = require("mongoose");
mongoose.set("debug", true);
mongoose.Promise = Promise;
// mongoose.connect("mongodb://mongo:27017/api-data", { useNewUrlParser: true,
//   keepAlive: true
// });
const option = {
    useNewUrlParser: true,
    socketTimeoutMS: 30000,
    
    keepAlive: true,
    reconnectTries: 30000
};

mongoose.connect('mongodb://localhost:27017/api-data', option).then(function(){
    //connected successfully
    console.log('Successfully connected to database');
}, function(err) {
    //err handle
    console.log('Not connected to database ' + err);
});


module.exports.User = require("./user");
module.exports.Message = require("./message");
