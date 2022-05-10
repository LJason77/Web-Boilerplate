print('Start #################################################################');

db = db.getSiblingDB('web');
db.createUser({
    user: 'web', pwd: 'web', roles: [{role: 'dbOwner', db: 'web'}],
},);

db.users.drop();
db.users.createIndex({"name": 1}, {unique: true, background: true});
db.users.insertOne({name: 'admin', password: 'admin'});

db.logs.drop();
db.logs.createIndex({"user": 1}, {background: true});
db.logs.createIndex({"status": 1}, {background: true});

print('END #################################################################');
