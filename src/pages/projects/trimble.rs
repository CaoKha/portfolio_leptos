use leptos::*;

#[component]
pub fn Trimble() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        Sensor Synchronization
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        Trimble SAS, Nantes, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200 ">6/2019</p>
                    <br/>
                    <img src="/images/trimble_robot.png" class="m-auto lg:w-1/3"/>
                    <p class="text-center dark:text-slate-400 sm:text-xl pt-4 lg:pt-10">
                        Revan is a camera-vision-based project intended for implementation on a forklift. 
                        The project aims to achieve localization with an error margin of less than 10 cm in both indoor and 
                        outdoor environments. To meet this goal, a robot has been constructed utilizing a combination of sensors 
                        including a camera, an IMU, a LiDAR, and wheel encoders. These sensors are then integrated to improve the accuracy 
                        of the localization results.
                        My role in this project involves sensor hardware synchronization, which entails synchronizing 
                        the data received from the camera (D435) and the data received from the IMU (DMU11). 
                        This synchronization process ensures that the data from both sensors are aligned, thereby reducing errors 
                        in the sensor fusion method.
                    </p>
                </div>
            </section>
            <section>
                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            1: Synchronize the camera and IMU by aligning the pulses from the IMU with those of the camera
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            To measure the signal from the IMU, 
                            I utilized an oscilloscope to capture the output from the SYNC pin (pin 8) of the IMU
                        </p>
                        <br/>
                    </div>
                    <div class="pt-1 lg:pt-0 block m-auto">
                        <img src="/images/trimble_imu_cam.png" class="aspect-auto"/>
                    </div>
                    <div>

                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            The signal obtained from pin 8 was not the expected 200 Hz output data rate (ODR) signal, 
                            but rather a 1000 Hz pulse signal. The pulse signal had a width of 0.19 ms. 
                            This discrepancy was not unexpected, as it was clearly stated in the datasheet.
                        </p>
                    </div>
                    <div class="pt-5 lg:pt-0 block m-auto">
                        <img src="/images/pin_info.png" class="aspect-auto"/>
                    </div>
                    <div>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            The 1000 Hz pulses indicate that the raw data is available inside the DMU11 chip. 
                            This raw data is subsequently filtered and transmitted to the computer via a USB port at a rate 
                            of 200 Hz. However, since 1000 is not divisible by the frame rates of the camera (15, 30, 60, or 90), 
                            which all contain a factor of 3, hardware synchronization between these signals cannot be achieved.
                        </p>
                    </div>
                    <div class="pt-5 lg:pt-0 block m-auto">
                        <img src="/images/imu_info.png" class="aspect-auto"/>
                    </div>

                </div>

                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            2: Phase locked loop
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Initially, we assumed that sending an electrical pulse to the camera would prompt it to capture an image. 
                            However, the reality is more complex. A camera doesn't function as a simple trigger-shoot device; 
                            instead, it's designed to maintain its frame rate at specific frequencies. That's why the datasheet 
                            recommends sending pulses to pin 8 at the exact frequency at which the camera is running.
                            
                            The camera employs a phase-locked loop technique, comparing the phase of an external signal with 
                            its own and adjusting the phase of its output signal accordingly. There's no option to configure the camera 
                            to operate as a trigger-shoot device. Therefore, we had to adjust our approach. Our new strategy involves 
                            precisely timing the pulses and then interpolating the data accordingly."
                        </p>
                        <br/>
                    </div>
                    <div class="pt-5 block m-auto lg:pt-0">
                        <img src="/images/phase_locked_loop.png" class="aspect-auto"/>
                    </div>

                    <div>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Risks when timestamping pulses:
                            Data is transferred from one device to another in three steps:"
                        </p>
                        <ol class="dark:text-slate-400 sm:text-xl pl-8">
                            <li>"• Logging (Data-ready)"</li>
                            <li>"• Sending"</li>
                            <li>"• Receiving"</li>
                        </ol>
                        <p class="dark:text-slate-400 sm:text-xl">
                            "Pulses indicate that the data is ready to be sent but do not serve as an indicator of the event \"data received\"."
                        </p>
                    </div>
                    <div class="pt-5 lg:pt-0 block m-auto">
                        <img src="/images/syncing.png" class="aspect-auto"/>
                    </div>
                </div>

                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            3: GNSS
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "To validate my assertion, I utilized a GNSS device as my clock to timestamp the pulses received 
                            from the camera. Additionally, I developed a testing code in C++ that timestamped images each time 
                            they were received from the camera, using a timing library. Upon observation, I detected inconsistencies 
                            between the timestamps in milliseconds obtained from the GNSS and those from the C++ library.
                            Given that the camera was configured to operate at 30 Hz, indicating that a frame should be received 
                            approximately every 33.33 ms, I analyzed the plotted graph using Octave. I noticed irregular gaps of 
                            66.66 ms between the data timestamps obtained from the C++ library. Conversely, the pulses timestamped 
                            with the GNSS exhibited a consistent, evenly distributed time spacing. This observation led to the conclusion 
                            that drop frame events had occurred.
                            To elaborate, the camera was initially set to \"auto exposure\". Consequently, when the robot entered a dark 
                            environment, it took time for the camera to adjust the frame before exporting it.
                            In conclusion, timestamping the pulses proved to be an unreliable method for ensuring data reception due to 
                            its susceptibility to drop frame events"
                        </p>
                        <br/>
                    </div>
                    <div class="pt-5 lg:pt-0 block m-auto">
                        <img src="/images/gnss.png" class="aspect-auto"/>
                    </div>
                </div>

                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            3: Interpolation
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "To obtain data from both the camera and IMU simultaneously without synchronization, 
                            interpolation appeared to be a potential solution worth considering. However, implementing and 
                            ensuring real-time functionality with this method posed a challenge.
                            Interpolation involves generating new data points within the range of a discrete set of known data points. 
                            Since we were dealing with time, which always progresses forward, halting the program to retrospectively 
                            create new data points was not feasible. Initially, it might seem that interpolation was not a real-time 
                            method suitable for this application.
                            However, the ROS data handling structure offered a solution to this challenge. Notably, timestamps were 
                            assigned to the data before being sent to ROS. As a result, we could manipulate both the data and its timestamps 
                            before transmitting them to ROS. This allowed us to effectively address the issue of real-time functionality 
                            in implementing the interpolation method." <br/>
                        </p>
                        <br/>
                    </div>
                    <div class="pt-5 lg:pt-0">
                        <img src="/images/interpolation.png" class="aspect-auto"/>
                    </div>
                </div>
                <div class="mx-5 pt-10 lg:mx-10">
                    <p class="dark:text-slate-400 sm:text-xl">
                        "The algorithm can be summarized as follows:" <ul class="pl-8">
                            <li>
                                "• Detect the data and timestamp of the IMU right before receiving an image."
                            </li>
                            <li>"• Upon receiving each image, mark its timestamp."</li>
                            <li>
                                "• Wait until the next data from the IMU is received and mark its timestamp as well."
                            </li>
                            <li>"• Create new IMU data at the time the image was received."</li>
                            <li>"• Repeat this process until the program concludes."</li>
                        </ul>
                        "Although initially challenging due to the different update rates of the BNO055 IMU (100 Hz) and the Realsense D435 camera set at 30 Hz, 
                        successful implementation was achieved. Despite initial data chaos upon sending to ROS, adjusting the method 
                        of data publication resolved the disorder, ensuring correct data order."
                    </p>
                </div>
                <div class="mx-5 pt-10 lg:mx-10">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            4: Conclusion
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Calibration and synchronization are two crucial techniques in sensor applications. 
                            The aim of this project is to synchronize the camera and IMU sensors. Initially, it was believed that 
                            using electrical pulses from the IMU to trigger the camera would solve the issue. However, experiments 
                            revealed that the camera's operation did not support this method. To address this, a pulse generator was 
                            employed to synchronize the operation of both the camera and IMU. However, the design of the IMU prevented it 
                            from being configured as a slave to the pulse generator. Consequently, the only remaining option was 
                            to use high-level programming languages such as C++ and ROS to manipulate the data and timestamps of each sensor. 
                            Although this method provided synchronization, its accuracy was impacted by latency and data throughput.
                            Initially, software-based approaches were deemed imprecise due to the limitations of non-real-time 
                            operating systems for timing. However, multitasking was necessary for this application, and the operating 
                            system's support for multithreaded programming proved efficient. Despite the sensors' unpredictable frequency 
                            characteristics, an interpolation method was proposed and successfully implemented in soft real time. 
                            While this resolved the synchronization problem, further optimization is required to maximize precision and 
                            stability."
                        </p>
                    </div>
                </div>
            </section>

        </main>
    }
}
