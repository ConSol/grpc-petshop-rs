module.exports = {
  apps: [
    {
      script: 'npm run dev',
      autorestart: false,
      shutdown_with_message: true,
    },
    {
      script: './scripts/protogen.sh',
      watch: '../server/proto',
    },
  ],
}
