import { useEffect, useState } from "react"
import PostCard from "./PostCard";

const CONTRACT_ID = process.env.REACT_APP_CONTRACT_NAME;

export default function PostPreview({ posts }) {
    
    return (
        <>
            {posts.map((res,i)=><PostCard key={res.post_id} post={res.post} />)}
        </>
    )
}