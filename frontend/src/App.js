import { useState } from 'react'
import PostPreview from './components/PostPreview';

const CONTRACT_ID = process.env.REACT_APP_CONTRACT_NAME;

function App({ wallet, isSignedIn }) {

    const [posts, setPosts] = useState([])

    const getPosts = () => wallet.viewMethod({ contractId: CONTRACT_ID, method: "get_posts" }).then(setPosts).catch(console.log)
    const writePost = () => {
        if (!isSignedIn)
            wallet.signIn()
        const args = {
            category: "test_category", title: "test_title", content: "test_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_contenttest_content" }
        wallet.callMethod({ contractId: CONTRACT_ID, method: "write_post", args: args }).then(getPosts).catch(console.log)
    }

    getPosts()

    return (
        <>
            <PostPreview posts={posts} />
            <button onClick={writePost}>Write Post</button>
            {isSignedIn && <button onClick={() => wallet.signOut()}>Sign Out</button>}
            {!isSignedIn && <button onClick={() => wallet.signIn()}>Sign In</button>}
        </>
    );
}

export default App;