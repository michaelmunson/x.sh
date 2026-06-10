import {createEffect, createProps, DIV, P, BUTTON} from "oxidizer"


const Counter = () => {
    const props = createProps({count: 0}, [
        // keep count greater than or equal to 0
        createEffect(['count'], $ => {
            if ($.count < 0){
                $.count = 0;
            }
        })
    ]);

    return (
        DIV(props, $ => [
            {id: 'counter-app'},
            P('Count: ' + $.count),
            DIV({style: {display: 'flex'}}, 
                BUTTON({onclick: () => $.count -= 1}, "Decrement"),
                BUTTON({onclick: () => $.count += 1}, "Increment")
            )
        ])
    )
}

export default function App(){
    return (
        DIV({id: 'app'},
            Counter()
        )
    )
}
