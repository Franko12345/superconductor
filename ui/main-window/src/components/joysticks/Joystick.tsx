import React from "react";
import {Draggable} from "react-beautiful-dnd";
import { connect, ConnectedProps } from "react-redux";
import { DriverStationState } from "../../store";
import { JoystickDataUpdate } from "../../ipc";

export type JoystickData = {
    name: string;
    id: string;
}

type OwnProps = JoystickData & {
    index: number;
}

const mapState = (state: DriverStationState, ownProps: OwnProps) => ({
    data: state.joystickData[ownProps.id] as JoystickDataUpdate | undefined
})

const connector = connect(mapState);
type Props = ConnectedProps<typeof connector> & OwnProps;

class JoystickComponent extends React.Component<Props, any> {
    render() {
        const { data } = this.props;

        return (
            <Draggable key={this.props.id} draggableId={this.props.id} index={this.props.index}>
                {(provided: any) => (
                    <div className="rounded bg-secondary d-flex flex-column border border-dark mb-2"
                         ref={provided.innerRef}
                         {...provided.draggableProps}
                         {...provided.dragHandleProps}>
                        <div className="d-flex w-100 p-2">
                            <p className="align-self-center m-0 flex-grow-1">{this.props.index}: {this.props.name}</p>
                        </div>
                        {data && (
                            <div className="p-2 pt-0" style={{ fontSize: "0.8em" }}>
                                <div className="d-flex flex-wrap mb-1">
                                    {data.axes.map((val, i) => (
                                        <div key={i} className="text-white bg-dark px-1 rounded mr-1 mb-1">A{i}: {val.toFixed(2)}</div>
                                    ))}
                                </div>
                                <div className="d-flex flex-wrap">
                                    {data.buttons.map((val, i) => (
                                        <div key={i} className={`px-1 rounded mr-1 mb-1 ${val ? 'bg-success text-white' : 'bg-dark text-white'}`}>B{i}</div>
                                    ))}
                                    {data.povs.map((val, i) => (
                                        <div key={`pov${i}`} className="px-1 rounded mr-1 mb-1 bg-info text-white">POV: {val}</div>
                                    ))}
                                </div>
                            </div>
                        )}
                    </div>
                )}
            </Draggable>
        );
    }
}

export const Joystick = connector(JoystickComponent);