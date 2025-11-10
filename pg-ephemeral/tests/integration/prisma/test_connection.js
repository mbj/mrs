import { PrismaClient } from '@prisma/client';

// DATABASE_URL should be set by pg-ephemeral run-env
const databaseUrl = process.env.DATABASE_URL;

if (!databaseUrl) {
  console.error('ERROR: DATABASE_URL environment variable is not set');
  process.exit(1);
}

console.log(`Connecting to: ${databaseUrl}`);

const prisma = new PrismaClient();

try {
  // Test the connection with a simple query
  await prisma.$queryRaw`SELECT 1`;

  console.log('SUCCESS: Connected to PostgreSQL successfully');

  await prisma.$disconnect();
} catch (error) {
  console.error('ERROR: Failed to connect to PostgreSQL:', error);
  await prisma.$disconnect();
  process.exit(1);
}
