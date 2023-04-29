/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  async rewrites() {
    return {
      fallback: [
        {
          source: "/shut-the-box/:path*",
          destination: "http://127.0.0.1:8383/shut-the-box/:path*", // Proxy to Backend
        },
      ],
    };
  },
};

module.exports = nextConfig;
