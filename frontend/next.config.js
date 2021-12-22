/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  async rewrites() {
    const port = parseInt(process.env.PORT);
    const backendPort = port + 100;

    return [
      {
        source: "/graphql",
        destination: `http://localhost:${backendPort}/graphql`,
      },
    ];
  },
};
