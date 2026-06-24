git clone https://github.com/commetchat/element-call.git element-call
cd element-call
pnpm install
pnpm build:sdk

cd ..
rm -r ./src/matrixrtc
cp -R ./element-call/dist/ ./src/matrixrtc
