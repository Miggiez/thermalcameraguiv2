import { useState, useEffect } from "react"
import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"
import { z } from "zod"

interface Payload {
	message: string
}

// const CAMERA_STATE = {
// 	CAMERAON: "CAMERAON",
// 	CAMERAOFF: "CAMERAOFF",
// } as const

// type CameraState = keyof typeof CAMERA_STATE

function App() {
	const [image, setImage] = useState<string>("")
	const [cameraState, setCameraState] = useState<boolean>(false)

	const handleStart = async () => {
		invoke("start").then((message) =>
			setCameraState(z.boolean().parse(message))
		)
	}

	const handleStop = async () => {
		invoke("stop").then((message) => setCameraState(z.boolean().parse(message)))
	}

	useEffect(() => {
		if (cameraState == true) {
			invoke("image_sending")
		}
	}, [cameraState])

	useEffect(() => {
		if (cameraState == true) {
			const unlisten = listen("image-byte", (event) => {
				let payload: Payload

				payload = z.object({ message: z.string() }).parse(event.payload)
				console.log(payload.message)
				setImage(payload.message)
			})
			return () => {
				unlisten.then((f) => f())
			}
		}
	}, [cameraState])

	return (
		<div>
			<h1 className="text-center mt-2">
				Camera is: {cameraState ? "ON!" : "OFF!"}
			</h1>
			<div className="flex justify-center items-center">
				{cameraState ? (
					<img src={image} width="400" height="400" />
				) : (
					<div className=" bg-slate-500 w-[300px] h-[300px]"></div>
				)}
			</div>

			<div className="flex justify-around items-center mt-4">
				<button className=" bg-slate-500 text-white p-3" onClick={handleStart}>
					start
				</button>
				<button className=" bg-slate-500 text-white p-3" onClick={handleStop}>
					stop
				</button>
			</div>
		</div>
	)
}

export default App
