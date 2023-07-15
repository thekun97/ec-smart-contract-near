npm run build:contract
npm run build:shop
npm run deploy:contract
npm run deploy:shop


# Shop smart contract
near call <accountId> new --accountId <accountId>
near call <accountId> add_shop '{id: "123", "name": "kunkunshop", "location": "HCM"}' --accountId <accountId>
near call <accountId> get_shop_by_owner '{"account_id": "<accountId>"}' --accountId <accountId>

