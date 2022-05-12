import { FC } from "react";
import { IStack } from "./IStack";

interface Props {
  stack: IStack;
}

const Stack: FC<Props> = ({ stack }) => {
  return (
    <div>
      <h1>{stack.key}</h1>
      <ul>
        {stack.techs.map((t) => (
          <li>{t.key}</li>
        ))}
      </ul>
    </div>
  );
};

export default Stack;
