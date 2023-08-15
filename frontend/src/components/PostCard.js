import './PostCard.css'; // 스타일 파일을 import합니다.

export default function PostCard({ post }) {
    return (
        <div className="post-card">
            <h2 className="post-title">{post.title}</h2>
            <span className="post-category">{post.category}</span>
            <span className="post-date">{new Date(post.created_at).toLocaleDateString()}</span>
            <div className="post-content-container">
                <p className="post-content">{post.content}</p>
            </div>
        </div>
    );
}
