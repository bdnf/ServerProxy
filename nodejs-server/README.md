
#Initialize new express app
docker build -t node_server -f Dockerfile.server .
docker run --name app -it node_server

npm init  
#install dependensies
npm install express bcrypt body-parser mongoose jsonwebtoken cors dotenv

#make server reflect changes interactively
npm install -g nodemon

#start server
node index.js or nodemon


#test
http POST localhost:1331/api/auth/signup username=testname8 password=paswrd email=testm8@email.com
http POST localhost:1331/api/auth/signin password=paswrd email=testm8@email.com
http POST localhost:1331/api/users/{}/messages "Authorization:Bearer {}" text="{}"
#read all messages
http GET localhost:1331/api/messages "Authorization:Bearer {}" || http GET localhost:1331/api/messages/showall
#test route for reading only messages
http GET localhost:1331/test # list of all entries
#change credentials
http GET localhost:1331/user/{}
http PUT localhost:1331/user/{} profileAccessLevel=admin
