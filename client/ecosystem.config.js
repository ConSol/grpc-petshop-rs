module.exports = {
  apps: [
    {
      script: 'npm run dev',
    },
    {
      script: './scripts/protogen.sh',
      watch: '../server/proto',
    },
  ],
}
