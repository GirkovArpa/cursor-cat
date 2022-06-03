import { $, $$ } from '@sciter';
import { launch } from '@env';
import { spawn } from '@sys';

main();

async function main() {
  adjustWindow();
  showTrayIcon();
}

function adjustWindow() {
  const [width, height] = Window.this.screenBox('frame', 'dimension');
  Window.this.move(0, 0, width, height, true);
  spawn(['ahk/freeze.exe']);
  setInterval(() => (Window.this.isTopmost = true));
  return { width, height };
}

globalThis.move_circle = function (x, y) {
  $('#disc').style.left = x + 'px';
  $('#disc').style.top = y + 'px';
};

Window.this.on('trayiconclick', ({ data }) => {
  const [sx, sy] = Window.this.box('position', 'client', 'screen');
  const menu = document.$('menu#tray');
  const { screenX, screenY } = data;
  menu.popupAt(screenX - sx, screenY - sy, 2);
});

async function showTrayIcon() {
  Window.this.trayIcon({
    image: await Graphics.Image.load('this://app/png/32x32.png'),
    text: 'Cursor Cat: Customizable mouse highlighter!',
  });
}

$('#size').on('change', (evt, el) => {
  $('#disc').attributes.width = el.value;
  $('#disc').attributes.height = el.value;
});

$('#transparency').on('change', (evt, el) => {
  $('#disc').style.opacity = el.value / 100;
});

$('#color').on('change', (evt, el) => {
  $('#disc').style.filter = `hue-rotate(${el.value}deg)`;
});

$('(about)').on('click', () => {
  Window.this.modal({ url: 'this://app/html/about.html' });
});

$('(exit)').on('click', () => Window.this.close());
