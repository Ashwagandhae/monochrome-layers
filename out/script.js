function build() {
  let ids = [];
  let pos = getPlayerPos();
  for ([text1, text2] of JSON_INPUT) {
    ids.push(buildText(text1, pos, false));
    ids.push(buildText(text2, pos, true));
  }
  window.DELETE_LAST = () => {
    for (let id of ids)
      gimbuild('REMOVE_DEVICE', {
        id: id,
      });
  };
}

function buildText(text, pos, downshift) {
  let id = generateDeviceId();
  gimbuild('PLACE_DEVICE', {
    id,
    deviceTypeId: 'textBillboard',
    options: JSON.stringify({
      text: text.text,
      fontSize: 13,
      scope: 'global',
      googleFont: 'PT Mono',
      color: text.color,
      alpha: text.alpha,
      strokeThickness: 0,
      strokeColor: '#FFFFFF',
      rotation: 0,
      visibleOnGameStart: 'Yes',
      showWhenReceivingFrom: '',
      hideWhenReceivingFrom: '',
    }),
    x: pos.x,
    y: pos.y + (downshift ? 7 : 0) + text.rows_above * 7.0,
  });
  return id;
}

function generateDeviceId() {
  const chars =
    'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let uniqueId = '';
  for (let i = 0; i < 21; i++) {
    const randomIndex = Math.floor(Math.random() * chars.length);
    uniqueId += chars[randomIndex];
  }
  return uniqueId;
}

function getPlayerPos(pos) {
  return GL?.stores?.phaser?.mainCharacter?.body;
}
