db.createUser(
    {
        user : "sgudim",
        pwd: "simon",
        roels: [
            {
                role : "readWrite",
                db : "indexer"
            }
        ]
    }
)