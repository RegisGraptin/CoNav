import { ConnectButton } from "@rainbow-me/rainbowkit";

export default function Header () {
    return <>
        <nav className="block w-full max-w-screen-lg my-4 px-4 py-2 mx-auto bg-white bg-opacity-90 sticky top-3 shadow lg:px-8 lg:py-3 backdrop-blur-lg backdrop-saturate-150 z-[9999]">
            <div className="container flex flex-wrap items-center justify-between mx-auto text-slate-800">
                <a href="/"
                className="mr-4 block cursor-pointer py-1.5 text-base text-slate-800 font-semibold">
                    CoNav
                </a>
                <ConnectButton />
            </div>
        </nav>
    </>
}