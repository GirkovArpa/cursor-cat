import { $, $$ } from '@sciter';
import { launch } from '@env';

document.on('click', 'a', (evt, el) => {
  const { href } = evt.target.attributes;
  const url = `${href.endsWith('/') ? '' : '/'}${href}?ref=cursor cat`;
  launch(url);
  return true;
});

$('button').on('click', () => Window.this.close());
