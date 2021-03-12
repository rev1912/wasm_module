const fastify = require('fastify')({
  logger: false
});

const path = require('path');

fastify.register(require('fastify-static'), {
  root: path.join(__dirname, 'dist'),
});

fastify.get('/', function (request, reply) {
  return reply.sendFile('index.html');
});

fastify.listen(3000, '0.0.0.0', function (err, address) {
  if (err) {
    fastify.log.error(err);
    process.exit(1);
  }
  console.log(`server listening on ${address}`);
});