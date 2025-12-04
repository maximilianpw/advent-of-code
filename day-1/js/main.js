import fs from 'fs';

function main() {
  const file = fs.readFileSync('../input.txt', 'utf-8').trim().split('\n');
  let position = 50;
  let password = 0;

  for (let line of file) {
    const distance = parseInt(line.slice(1));
    let original = position;

    switch (line[0]) {
      case 'R':
        position += distance;
        break;
      case 'L':
        position -= distance;
        break;
    }

    while (position < 0) {
      position += 100;
      if (original !== 0) {
        password += 1;
      }
      original = position;
    }

    let dejafait = false;
    while (position > 99) {
      position -= 100;
      password += 1;
      dejafait = true;
    }

    if (position === 0 && !dejafait) {
      password += 1;
    }
  }

  console.log(password);
}

main();

