print('Start #################################################################');

db = db.getSiblingDB('web');
db.createUser({
    user: 'web', pwd: 'web', roles: [{role: 'dbOwner', db: 'web'}],
},);

db.users.drop();
db.users.createIndex({"name": 1}, {unique: true, background: true});
db.users.insertOne({name: 'admin', password: 'c403b04d4617d99596164dbac8319d11'});

print('END #################################################################');
