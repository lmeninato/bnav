import React from "react";

import shipImages from "./../assets/spritesheet.png";

const ship_direction = {
  UP: {
    offsetX: -16,
    offsetY: -3,
    width: 23,
    height: 54,
  },
  RIGHT: {
    offsetX: -221,
    offsetY: -7,
    width: 53,
    height: 43,
  },
  DOWN: {
    offsetX: -456,
    offsetY: 0,
    width: 23,
    height: 59,
  },
  LEFT: {
    offsetX: -661,
    offsetY: -7,
    width: 53,
    height: 44,
  },
};

class Player extends React.Component {
  constructor(props) {
    super(props);

    this.images = shipImages;
    this.angles = Array.apply(null, { length: 16 }).map(
      (_, i) => (360 / 16) * i
    ); // angles of sprites

    this.state = {
      locX: props.startX,
      locY: props.startY,
      direction: props.direction,
    };
  }

  style() {
    let res = {
      height: "40px",
      width: "40px",
      margin: "auto",
    };

    return res;
  }

  spriteStyle() {
    let ship_dir = ship_direction[this.state.direction];

    let res = {
      background: `url(${this.images}) no-repeat ${ship_dir["offsetX"]}px ${ship_dir["offsetY"]}px`,
      width: `${ship_dir["width"]}px`,
      height: `${ship_dir["height"]}px`,
    };

    return res;
  }

  render() {
    return (
      <div className="Player" style={this.style()}>
        <img style={this.spriteStyle()}></img>
      </div>
    );
  }
}

export default Player;
